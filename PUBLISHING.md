# Publishing TBenni

The [`release.yml`](.github/workflows/release.yml) workflow builds abi3 wheels for
Linux (x86_64 and aarch64), macOS (Intel and Apple Silicon) and Windows, plus an
sdist. Pushing a tag such as `v0.1.0` also creates a GitHub Release with all
artifacts attached.

## One-time PyPI setup (trusted publishing)

PyPI can authenticate GitHub Actions through OpenID Connect, so no long-lived
API token is required.

1. Create an account on [pypi.org](https://pypi.org/account/register/) if needed.
2. Open <https://pypi.org/manage/account/publishing/> and add a **pending
   publisher** with these values:

   | Field | Value |
   | --- | --- |
   | PyPI Project Name | `tbenni` |
   | Owner | `NeonFeline` |
   | Repository name | `TBenni` |
   | Workflow name | `release.yml` |
   | Environment name | `pypi` |

3. Enable publishing in this repository:

   ```sh
   gh variable set PYPI_PUBLISH --body true -R NeonFeline/TBenni
   ```

4. Push a tag (or re-run the release workflow):

   ```sh
   git tag v0.1.0
   git push origin v0.1.0
   ```

The `pypi-publish` job only runs when the `PYPI_PUBLISH` repository variable is
`true`, so builds and GitHub Releases keep working before PyPI is configured.

## Alternative: API token

If you prefer an API token instead of trusted publishing, create one at
<https://pypi.org/manage/account/token/> and store it as a repository secret:

```sh
gh secret set PYPI_API_TOKEN --body "pypi-..." -R NeonFeline/TBenni
```

Then add this input to the `pypa/gh-action-pypi-publish` step in
`release.yml`:

```yaml
with:
  password: ${{ secrets.PYPI_API_TOKEN }}
```

## Verifying a release

```sh
python -m pip install --upgrade tbenni
python -c "import tbenni; print(tbenni.__version__, tbenni.phonemize('czysta', include_stress=False))"
```

The GitHub Release wheels can also be installed directly before the PyPI
project is live:

```sh
python -m pip install https://github.com/NeonFeline/TBenni/releases/download/v0.1.0/tbenni-0.1.0-cp38-abi3-manylinux_2_17_x86_64.manylinux2014_x86_64.whl
```
